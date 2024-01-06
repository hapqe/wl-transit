<script>
  import { GeoAlt } from "svelte-bootstrap-icons";
</script>

<div id="wrapper">
  <div id="heading">
    <h1>wl transit</h1>
  </div>
  <span id="location"><GeoAlt />Theresiengasse 54</span>
  <div id="departures">
    <span><b>U6</b> Floridsdorf • <b>2min</b> • 500m</span>
    <span><b>U6</b> Siebenhirten • <b>2min</b> • 500m</span>
    <span><b>U6</b> Floridsdorf • <b>2min</b> • 500m</span>
    <span><b>U6</b> Floridsdorf • <b>2min</b> • 500m</span>
    <span><b>U6</b> Floridsdorf • <b>2min</b> • 500m</span>
    <span><b>U6</b> Floridsdorf • <b>2min</b> • 500m</span>
    <span><b>U6</b> Floridsdorf • <b>2min</b> • 500m</span>
    <span><b>U6</b> Floridsdorf • <b>2min</b> • 500m</span>
  </div>
  <div></div>
  <div></div>
  <div></div>
</div>

<style>
  h1 {
    font-weight: 100;
    font-size: 2.5rem;
    margin-top: 0.5rem;
    margin-left: 1rem;
    margin-bottom: 0;
    opacity: 0.5;
  }

  #wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    height: 100%;
  }

  #heading {
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  #location {
    opacity: 0.5;
  }

  #departures {
    font-weight: 100;
    text-align: center;
    display: grid;
    gap: 1rem;
  }
</style>
