import { TextField, Button, Grid, Item } from "@mui/material";

export default function Home() {
  return (
    <div>
      <h1>Decimal to Binary Converter</h1>
      <Grid container spacing={2}>
        <br></br>
        <br></br>
        <br></br>
        <TextField id="outlined-basic" label="Decimal" variant="outlined" />
        <TextField id="outlined-basic" label="Binary" variant="outlined" disabled />
        <Button>Convert</Button>
      </Grid>
      <Grid container spacing={2}>
        <br></br>
        <br></br>
        <br></br>
        <TextField id="outlined-basic" label="Binary" variant="outlined"  />
        <TextField id="outlined-basic" label="Decimal" variant="outlined" disabled/>
        <Button>Convert</Button>
      </Grid>
    </div>
  );
}
