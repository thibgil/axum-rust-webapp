use axum::{extract::State, Json};
use crate::{model::{ModelController, Ticket, TicketForCreate}, Result};

async fn create_ticket(
    State(mc): State<ModelController>, Json(ticket_fc): Json<TicketForCreate>, 
) -> Result<Json<Ticket>> {
    println!("=> {:<12} - create_ticket", "HANDLER");
    
    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}
