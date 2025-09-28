
import express, { Request, Response } from "express";

const app = express();
const port = 3000;

app.get('/api', (req: Request, res: Response) => {
  //An API that returns a response with 90% probability or else fails
  const sample = Math.random();
  if (sample <= 0.9) {
    res.send({
      value: sample      
    });
  }
  res.status(400).end();
});

app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
});