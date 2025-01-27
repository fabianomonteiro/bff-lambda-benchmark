module "lambda_rust" {
  source = "git::https://github.com/fabianomonteiro/aws-lambda-module.git"

  region                = "us-east-1"
  function_name         = "my-rust-lambda"
  runtime               = "provided.al2"
  handler               = "bootstrap"
  iam_role_arn          = "arn:aws:iam::123456789012:role/lambda-role"
  deployment_package    = "path/to/bootstrap.zip" # Includes the compiled binary
  environment_variables = {
    ENV_VAR = "value"
  }
  timeout              = 30
  memory_size          = 128
  log_retention_days   = 14
}
