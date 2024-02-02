<script>
  import { GeoAlt, Line } from "svelte-bootstrap-icons";
  import init, { closest_stations } from "../pkg/station_picker";
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
    const stations = await closest_stations(pos.latitude, pos.longitude, 400)
      .split("\n")
      .filter((s) => s !== "")
      .map((s) => s.split(":"));

    const data = await Promise.all(
      stations.map((s) => fetch("/ogd_realtime/monitor?diva=" + s[0])),
    );

    const json = await Promise.all(data.map((data) => data.json()));

    return {
      json,
      stations: stations.map((s) => s[1]),
      pos: [pos.longitude, pos.latitude],
    };
  }
  info();
</script>

{#await info() then { stations, json, pos }}
  <div id="heading">
    <h1>wl transit</h1>
  </div>
  {#each json as { data: { monitors } }, i}
    <div transition:fly={{ y: 100 }} id="location">
      <span id="station"><GeoAlt />{stations[i]}</span>
      <div id="departures">
        {#each monitors as { locationStop: { geometry: { coordinates } }, lines: [line] }}
          <span>
            <b>{line.name}</b>
            {line.towards} •
            <b>
              {line.departures.departure
                .map((d) => d.departureTime.countdown)
                .slice(0, 3)
                .join(", ")}min
            </b>
            • {distance(pos, coordinates)}
          </span>
        {/each}
      </div>
    </div>
  {/each}
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

  #location {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 2rem;
  }

  #heading {
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  #station {
    opacity: 0.5;
    margin-bottom: 0.5rem;
  }

  #departures {
    font-weight: 100;
    text-align: center;
    display: grid;
    gap: 1rem;
  }
</style>
