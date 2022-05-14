require("dotenv").config({ path: "./.env" });
// prettier-ignore
import {
  Stack,
  StackProps,
  aws_lambda as lambda,
  aws_dynamodb as ddb,
  aws_dynamodb,
  Duration,
} from "aws-cdk-lib";
import { Cors, LambdaIntegration, PassthroughBehavior, RestApi } from "aws-cdk-lib/aws-apigateway";
import { Construct } from "constructs";
export class SitrepEventsWrapperStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const fn = new lambda.Function(this, "SitrepEventsCdkFunc", {
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: "hello.hanlder",
      code: lambda.Code.fromAsset(process.env.RELEASE_FILE ? process.env.RELEASE_FILE : ""),
      timeout: Duration.seconds(5),
      memorySize: 2048,
      architecture: lambda.Architecture.X86_64,
    });

    try {
      const ddbEventsTable = new ddb.Table(this, "SitrepEvents", {
        tableName: "sitrep-events",
        partitionKey: {
          name: "PK",
          type: aws_dynamodb.AttributeType.STRING,
        },
        sortKey: {
          name: "SK",
          type: aws_dynamodb.AttributeType.STRING,
        },
      });
      ddbEventsTable.grantFullAccess(fn);
    } catch (err) {
      console.log("table already exists");
    }

    const sitrepEventsApi = new RestApi(this, "sitrep-events-api", {
      defaultCorsPreflightOptions: {
        allowOrigins: Cors.ALL_ORIGINS,
      },
    });

    const sitrepEventItems = sitrepEventsApi.root.addResource("event", {
      defaultCorsPreflightOptions: {
        allowOrigins: ["*"],
      },
    });

    const sitrepEventsGETIntegration: LambdaIntegration = new LambdaIntegration(fn, {
      proxy: false,
      requestTemplates: {
        "application/json":
          '{\r\n\
            "command": "$input.params(\'command\')"\r\n\
          }',
      },
      integrationResponses: [
        {
          statusCode: "200",
        },
      ],
    });

    const sitrepEventsPOSTIntegration: LambdaIntegration = new LambdaIntegration(fn, {
      proxy: false,
      requestTemplates: {
        "application/json":
          "{\r\n\
            \"command\": $input.json('$.command'),\r\n\
            \"headline\": $input.json('$.headline'),\r\n\
            \"event_type\": $input.json('$.eventType'),\r\n\
            \"read_by\": $input.json('$.readBy'),\r\n\
            \"severity\": $input.json('$.severity'),\r\n\
            \"scope\": $input.json('$.scope'),\r\n\
            \"status\": $input.json('$.status'),\r\n\
            \"step\": $input.json('$.step')\r\n\
          }",
      },
      passthroughBehavior: PassthroughBehavior.WHEN_NO_TEMPLATES,
      integrationResponses: [{ statusCode: "200" }],
    });

    sitrepEventItems.addMethod("GET", sitrepEventsGETIntegration, {
      apiKeyRequired: false,
      methodResponses: [
        {
          statusCode: "200",
          responseParameters: {
            "method.response.header.Content-Type": true,
          },
        },
      ],
    });

    sitrepEventItems.addMethod("POST", sitrepEventsPOSTIntegration, {
      apiKeyRequired: false,
      methodResponses: [{ statusCode: "200" }],
    });
  }
}
