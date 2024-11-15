use crate::models::relays::Relays;

pub enum RelayCommand {
    Add(Relays),
    Remove(Relays),
}