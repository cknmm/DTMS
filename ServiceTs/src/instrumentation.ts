/*instrumentation.ts*/
import { NodeSDK } from '@opentelemetry/sdk-node';
//import { ConsoleSpanExporter } from '@opentelemetry/sdk-trace-node';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-proto';
import { getNodeAutoInstrumentations } from '@opentelemetry/auto-instrumentations-node';
import {
  PeriodicExportingMetricReader,
  // ConsoleMetricExporter,
} from '@opentelemetry/sdk-metrics';
import { OTLPMetricExporter } from '@opentelemetry/exporter-metrics-otlp-proto';

const sdk = new NodeSDK({
  traceExporter: new OTLPTraceExporter({
    headers: {}
  }),
  metricReader: new PeriodicExportingMetricReader({
    exporter: new OTLPMetricExporter({
      headers: {},
      concurrencyLimit: 1
    }),
  }),
  instrumentations: [getNodeAutoInstrumentations()],
});

sdk.start();
