import random
from fastapi import FastAPI, HTTPException

app = FastAPI()

@app.get('/api')
async def api_call():
    chance: float = random.random()
    #80% chance that the api suceeds and returns a valid response otherwise 400
    if chance <= 0.8:
        return { "value": chance }
    raise HTTPException(status_code=400, detail='Could not sample a success value!')