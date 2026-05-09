"use client";

import { WebTracerProvider, BatchSpanProcessor } from "@opentelemetry/sdk-trace-web";
import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-http";
import { registerInstrumentations } from "@opentelemetry/instrumentation";
import { FetchInstrumentation } from "@opentelemetry/instrumentation-fetch";
import { Resource } from "@opentelemetry/resources";
import { SemanticResourceAttributes } from "@opentelemetry/semantic-conventions";

/**
 * Initializes OpenTelemetry tracing for the web frontend.
 */
export const initTracing = () => {
  if (typeof window === "undefined") return;

  const exporter = new OTLPTraceExporter({
    url: "http://localhost:4318/v1/traces",
  });

  const provider = new WebTracerProvider({
    resource: new Resource({
      [SemanticResourceAttributes.SERVICE_NAME]: "leave-web-frontend",
    }),
  });

  provider.addSpanProcessor(new BatchSpanProcessor(exporter));

  registerInstrumentations({
    instrumentations: [
      new FetchInstrumentation({
        propagateTraceHeaderCorsUrls: [
          /http:\/\/localhost:8080\/.*/, // Propagate to API Gateway
        ],
      }),
    ],
  });

  provider.register();
  console.log("🔭 OTel Tracing Initialized: leave-web-frontend");
};
