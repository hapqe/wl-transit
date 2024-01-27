<script>
  import { GeoAlt } from "svelte-bootstrap-icons";
  import init, { closest_station } from "../pkg/station_picker";
  import { fly } from "svelte/transition";

  function getCurrentPosition() {
    return new Promise((resolve, reject) => {
      if (!navigator.geolocation) {
        alert("Standortbestimmung wird nicht unterstützt");
        reject(new Error("Geolocation is not supported by your browser"));
      } else {
        navigator.geolocation.getCurrentPosition(
          (position) => resolve(position.coords),
          (error) => {
            alert("Standortbestimmung fehlgeschlagen");
            return reject(error);
          },
        );
      }
    });
  }

  function distance(a, b) {
    const deg2rad = Math.PI / 180;
    const lat1 = a[0] * deg2rad;
    const lon1 = a[1] * deg2rad;
    const lat2 = b[0] * deg2rad;
    const lon2 = b[1] * deg2rad;

    return (
      Math.round(
        6371000 *
          Math.acos(
            Math.cos(lat1) * Math.cos(lat2) * Math.cos(lon2 - lon1) +
              Math.sin(lat1) * Math.sin(lat2),
          ),
      ) + "m"
    );
  }

  async function info() {
    const [_, pos] = await Promise.all([init(), getCurrentPosition()]);
    const station = await closest_station(pos.latitude, pos.longitude);

    const [id, location] = station.split(":");

    const data = await fetch("/ogd_realtime/monitor?diva=" + id);
    const json = await data.json();
    console.log(json);
    const lines = json.data.monitors;

    return { lines, location, pos: [pos.longitude, pos.latitude] };
  }
</script>

{#await info() then { location, lines: all, pos }}
  <div transition:fly={{ y: 100 }} id="wrapper">
    <div id="heading">
      <h1>wl transit</h1>
    </div>
    <span id="location"><GeoAlt />{location}</span>
    <div id="departures">
      {#each all as { lines, locationStop }}
        <span
          ><b>{lines[0].name}</b>
          {lines[0].towards} •
          <b>
            {lines[0].departures.departure
              .map((d) => d.departureTime.countdown)
              .slice(0, 3)
              .join(", ")}min
          </b>
          • {distance(locationStop.geometry.coordinates, pos)}</span
        >
      {/each}
    </div>
    <div></div>
    <div></div>
    <div></div>
  </div>
{/await}

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
