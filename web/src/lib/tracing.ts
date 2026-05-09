"use client";

import { WebTracerProvider, BatchSpanProcessor } from "@opentelemetry/sdk-trace-web";
import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-http";
import { registerInstrumentations } from "@opentelemetry/instrumentation";
import { FetchInstrumentation } from "@opentelemetry/instrumentation-fetch";
import { resourceFromAttributes } from "@opentelemetry/resources";
import { ATTR_SERVICE_NAME } from "@opentelemetry/semantic-conventions";

/**
 * Initializes OpenTelemetry tracing for the web frontend.
 */
export const initTracing = () => {
  if (typeof window === "undefined") return;

  const exporter = new OTLPTraceExporter({
    url: "http://localhost:4318/v1/traces",
  });

  const provider = new WebTracerProvider({
    resource: resourceFromAttributes({
      [ATTR_SERVICE_NAME]: "leave-web-frontend",
    }),
    spanProcessors: [new BatchSpanProcessor(exporter)],
  });

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
  console.log("🔭 OTel Tracing Initialized: leave-web-frontend (v2.x)");
};
