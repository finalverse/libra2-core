script {
  use libra2_framework::libra2_governance;
  use libra2_framework::staking_config;

  fun main(proposal_id: u64) {
    let framework_signer = libra2_governance::resolve(proposal_id, @libra2_framework);
    // Change recurring lockup to 1 day.
    let one_day_in_secs = 24 * 60 * 60;
    staking_config::update_recurring_lockup_duration_secs(&framework_signer, one_day_in_secs);
  }
}
