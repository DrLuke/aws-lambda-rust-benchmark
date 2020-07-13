from aws_cdk import (
    core,
    aws_lambda,
    aws_apigateway
)


class Stack(core.Stack):
    def __init__(self, scope, id, *args, **kwargs):
        super(Stack, self).__init__(scope, id, *args, **kwargs)

        self.function = aws_lambda.Function(self, "Function",
                                            runtime=aws_lambda.Runtime.PROVIDED,
                                            timeout=core.Duration.minutes(1),
                                            tracing=aws_lambda.Tracing.ACTIVE,
                                            code=aws_lambda.AssetCode("lambda"),
                                            handler="irr.elevant")

        self.apigw = aws_apigateway.LambdaRestApi(self, "Api",
                                                  handler=self.function,
                                                  proxy=False)

        self.apigw.root.add_method("GET")
        self.apigw.root.add_method("POST")


app = core.App()

stack = Stack(app, "RustBenchmarkStack")

app.synth()
