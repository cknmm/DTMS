import random
from fastapi import FastAPI, HTTPException
from instrumentation import init_otel_and_instrument_app

app = FastAPI()

#instrument app
init_otel_and_instrument_app(app)

@app.get('/api')
async def api_call():
  chance: float = random.random()
  #80% chance that the api suceeds and returns a valid response otherwise 400
  if chance <= 0.8:
      return { "value": chance }
  raise HTTPException(status_code=400, detail='Could not sample a success value!')