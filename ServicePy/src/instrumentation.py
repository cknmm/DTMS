from opentelemetry.instrumentation.fastapi import FastAPIInstrumentor
from opentelemetry.sdk.resources import Resource
from opentelemetry import trace, metrics
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.metrics.export import PeriodicExportingMetricReader
from opentelemetry.exporter.otlp.proto.grpc.metric_exporter import OTLPMetricExporter
from opentelemetry.sdk.metrics import MeterProvider
from opentelemetry.semconv.attributes import service_attributes

from fastapi import FastAPI

#globals
SERVICE_NAME = "fastapiMicroservicePy"

def init_otel() -> tuple[TracerProvider, MeterProvider]:

  global SERVICE_NAME

  resource = Resource.create(attributes={
    service_attributes.SERVICE_NAME: SERVICE_NAME,
    service_attributes.SERVICE_VERSION: "1.0",
  })

  #trace provider
  tracer_provider = TracerProvider(resource=resource)
  trace.set_tracer_provider(tracer_provider)
  tracer_provider.add_span_processor(BatchSpanProcessor(OTLPSpanExporter()))

  #metrics providers
  metric_exporter = PeriodicExportingMetricReader(OTLPMetricExporter(endpoint="http://localhost:4317"))
  meter_provider = MeterProvider(resource=resource, metric_readers=[metric_exporter])
  metrics.set_meter_provider(meter_provider)

  #Add a request counter
  meter = metrics.get_meter(SERVICE_NAME)
  meter.create_counter(
    name="request_counter",
    description="Total number of http requests received",
    unit="{requests}"
  )

  return tracer_provider, meter_provider


def init_otel_and_instrument_app(app: FastAPI):
  tracer_provider, meter_provider = init_otel()
  FastAPIInstrumentor().instrument_app(app, tracer_provider=tracer_provider, meter_provider=meter_provider)