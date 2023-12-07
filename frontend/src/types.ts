export interface State {
  is_first_user: boolean
  public_registration: boolean
  recaptcha_config: RecaptchaConfig
  started_at: Date
}

export interface RecaptchaConfig {
  site_key: string
  require_on_registration: boolean
  require_on_login: boolean
  require_on_password_reset: boolean
}
