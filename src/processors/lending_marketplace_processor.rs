use std::fmt::Debug;
use std::sync::Arc;

use crate::config::ProcessorConfig;
use crate::processors::ProcessorTrait;
use crate::utils::timestamp_millis_to_naive_datetime;
use crate::{db::DbPool, types::BlockAndEvents};
use anyhow::Result;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::NaiveDateTime;
use diesel::expression::AsExpression;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sql_types::SmallInt;
use diesel_async::RunQueryDsl;
use diesel_enum::DbEnum;
use serde::Serialize;

use diesel::FromSqlRow;

#[derive(Queryable, Selectable, Insertable, Debug, Clone, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::loan_actions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoanActionModel {
    loan_subcontract_id: String,
    loan_id: Option<BigDecimal>,
    by: String,
    timestamp: NaiveDateTime,
    action_type: LoanActionType,
}

pub struct LendingContractProcessor {
    connection_pool: Arc<DbPool>,
    contract_address: String,
}

impl LendingContractProcessor {
    pub fn new(connection_pool: Arc<DbPool>, contract_address: String) -> Self {
        Self { connection_pool, contract_address }
    }
}

impl Debug for LendingContractProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "LoanActionProcessor {{ connections: {:?}  idle_connections: {:?} }}",
            state.connections, state.idle_connections
        )
    }
}

#[async_trait]
impl ProcessorTrait for LendingContractProcessor {
    fn name(&self) -> &'static str {
        ProcessorConfig::LendingContractProcessor("".into()).name()
    }

    fn connection_pool(&self) -> &Arc<DbPool> {
        &self.connection_pool
    }

    async fn process_blocks(
        &self,
        _from: i64,
        _to: i64,
        blocks: Vec<Vec<BlockAndEvents>>,
    ) -> Result<()> {
        // Process blocks and insert to db
        let models = convert_to_model(blocks, &self.contract_address);
        if !models.is_empty() {
            insert_to_db(self.connection_pool.clone(), models).await?;
        }
        Ok(())
    }
}

/// Insert loan actions into the database.
pub async fn insert_to_db(db: Arc<DbPool>, actions: Vec<LoanActionModel>) -> Result<()> {
    let mut conn = db.get().await?;
    insert_into(crate::schema::loan_actions::table).values(&actions).execute(&mut conn).await?;
    Ok(())
}

pub fn convert_to_model(
    blocks: Vec<Vec<BlockAndEvents>>,
    contract_address: &str,
) -> Vec<LoanActionModel> {
    let mut models = Vec::new();
    for bes in blocks {
        for be in bes {
            let events = be.events;
            for event in events {
                if event.contract_address.eq(&contract_address) {
                    if let Some(action) = LoanActionType::from_event_index(event.event_index) {
                        match action {
                            LoanActionType::LoanCreated => {
                                models.push(LoanActionModel {
                                    loan_subcontract_id: event.fields[0].value.clone(),

                                    action_type: action,
                                    by: event.fields[2].value.clone(),
                                    timestamp: timestamp_millis_to_naive_datetime(
                                        event.fields[3].value.parse::<i64>().unwrap(),
                                    ),
                                    loan_id: Some(
                                        BigDecimal::from_f64(
                                            event.fields[1].value.parse::<f64>().unwrap(),
                                        )
                                        .unwrap(),
                                    ),
                                });
                            }
                            _ => {
                                models.push(LoanActionModel {
                                    loan_subcontract_id: event.fields[0].value.clone(),
                                    action_type: action,
                                    by: event.fields[1].value.clone(),
                                    timestamp: timestamp_millis_to_naive_datetime(
                                        event.fields[2].value.parse::<i64>().unwrap(),
                                    ),
                                    loan_id: None, // Other actions does not need this field
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    models
}

#[derive(Debug, thiserror::Error)]
#[error("CustomError: {msg}, {status}")]
pub struct CustomError {
    msg: String,
    status: u16,
}

impl CustomError {
    fn not_found(msg: String) -> Self {
        Self { msg, status: 404 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromSqlRow, DbEnum, Serialize, AsExpression)]
#[diesel(sql_type = SmallInt)]
#[diesel_enum(error_fn = CustomError::not_found)]
#[diesel_enum(error_type = CustomError)]
pub enum LoanActionType {
    LoanCreated,
    LoanCancelled,
    LoanPaid,
    LoanAccepted,
    LoanLiquidated,
}

impl LoanActionType {
    pub fn from_event_index(event_index: i32) -> Option<Self> {
        match event_index {
            2 => Some(Self::LoanCreated),
            3 => Some(Self::LoanCancelled),
            4 => Some(Self::LoanPaid),
            5 => Some(Self::LoanAccepted),
            6 => Some(Self::LoanLiquidated),
            _ => None,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::LoanCreated => "LoanCreated".to_string(),
            Self::LoanCancelled => "LoanCancelled".to_string(),
            Self::LoanPaid => "LoanPaid".to_string(),
            Self::LoanAccepted => "LoanAccepted".to_string(),
            Self::LoanLiquidated => "LoanLiquidated".to_string(),
        }
    }
}
