module "lambda_python" {
  source = "git::https://github.com/fabianomonteiro/aws-lambda-module.git"

  region                = "us-east-1"
  function_name         = "my-python-lambda"
  runtime               = "python3.8"
  handler               = "lambda_function.lambda_handler"
  iam_role_arn          = "arn:aws:iam::123456789012:role/lambda-role"
  deployment_package    = "path/to/package.zip"
  environment_variables = {
    ENV_VAR = "value"
  }
  timeout              = 30
  memory_size          = 128
  log_retention_days   = 14
}
