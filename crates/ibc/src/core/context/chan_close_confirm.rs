use crate::prelude::*;

use crate::core::ics04_channel::channel::State;
use crate::core::ics04_channel::error::ChannelError;
use crate::core::ics04_channel::events::CloseConfirm;
use crate::core::ics04_channel::handler::chan_close_confirm;
use crate::core::ics04_channel::msgs::chan_close_confirm::MsgChannelCloseConfirm;
use crate::core::ics26_routing::context::ModuleId;

use crate::events::IbcEvent;

use super::{ContextError, ExecutionContext, ValidationContext};

pub(super) fn chan_close_confirm_validate<ValCtx>(
    ctx_b: &ValCtx,
    module_id: ModuleId,
    msg: MsgChannelCloseConfirm,
) -> Result<(), ContextError>
where
    ValCtx: ValidationContext,
{
    chan_close_confirm::validate(ctx_b, &msg)?;

    let module = ctx_b
        .get_route(&module_id)
        .ok_or(ChannelError::RouteNotFound)?;
    module.on_chan_close_confirm_validate(&msg.port_id_on_b, &msg.chan_id_on_b)?;

    Ok(())
}

pub(super) fn chan_close_confirm_execute<ExecCtx>(
    ctx_b: &mut ExecCtx,
    module_id: ModuleId,
    msg: MsgChannelCloseConfirm,
) -> Result<(), ContextError>
where
    ExecCtx: ExecutionContext,
{
    let module = ctx_b
        .get_route_mut(&module_id)
        .ok_or(ChannelError::RouteNotFound)?;
    let extras = module.on_chan_close_confirm_execute(&msg.port_id_on_b, &msg.chan_id_on_b)?;

    let chan_end_on_b = ctx_b.channel_end(&(msg.port_id_on_b.clone(), msg.chan_id_on_b.clone()))?;

    // state changes
    {
        let chan_end_on_b = {
            let mut chan_end_on_b = chan_end_on_b.clone();
            chan_end_on_b.set_state(State::Closed);
            chan_end_on_b
        };

        let port_channel_id_on_b = (msg.port_id_on_b.clone(), msg.chan_id_on_b.clone());
        ctx_b.store_channel(port_channel_id_on_b, chan_end_on_b)?;
    }

    // emit events and logs
    {
        ctx_b.log_message("success: channel close confirm".to_string());

        let core_event = {
            let port_id_on_a = chan_end_on_b.counterparty().port_id.clone();
            let chan_id_on_a = chan_end_on_b
                .counterparty()
                .channel_id
                .clone()
                .ok_or(ContextError::ChannelError(ChannelError::Other {
                description:
                    "internal error: ChannelEnd doesn't have a counterparty channel id in CloseInit"
                        .to_string(),
            }))?;
            let conn_id_on_b = chan_end_on_b.connection_hops[0].clone();

            IbcEvent::CloseConfirmChannel(CloseConfirm::new(
                msg.port_id_on_b.clone(),
                msg.chan_id_on_b.clone(),
                port_id_on_a,
                chan_id_on_a,
                conn_id_on_b,
            ))
        };
        ctx_b.emit_ibc_event(core_event);

        for module_event in extras.events {
            ctx_b.emit_ibc_event(IbcEvent::AppModule(module_event));
        }

        for log_message in extras.log {
            ctx_b.log_message(log_message);
        }
    }

    Ok(())
}
