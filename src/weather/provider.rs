use crate::weather::info::WeaterInfo;

trait Provider {
    fn get_info<I>(address: &str) -> I
    where 
    I: WeaterInfo;
}