
import 'mapbox-gl/dist/mapbox-gl.css';
import mapboxgl from 'mapbox-gl'; // or "const mapboxgl = require('mapbox-gl');"
// import turf from "@turf/turf"
// import { Quad, Point, pip } from "./pkg"
// import { Quad, Point, pip } from "./pkg"

let inite = [[
    -91.87645196914673,
    42.75867170869857
],
[
    -91.87158107757568,
    42.75867170869857
],
[
    -91.87158107757568,
    42.761665019319686
],
[
    -91.87645196914673,
    42.761665019319686
]]
async function main() {
    // await init()

    const { Quad, Point, pip } = await import("./pkg")
    // console.log(typeof turf)

    mapboxgl.accessToken = 'pk.eyJ1IjoiZ2FybmF2YXVyaGEiLCJhIjoiY2s5ZjV5YTg3MDEyNzNla2Z3cXZtbTB3NCJ9.8a1Uny-88cOa9MlPXIAJCg';
    const map = new mapboxgl.Map({
        container: 'map', // container ID
        style: 'mapbox://styles/mapbox/light-v10', // style URL
        center: [-91.874, 42.76], // starting position [lng, lat]
        zoom: 12 // starting zoom
    });
    map.on("load", () => {
        map.addSource('maine', {
            'type': 'geojson',
            'data': {
                'type': 'Feature',
                'geometry': {
                    'type': 'Polygon',
                    // These coordinates outline Maine.
                    'coordinates': [inite]
                }
            }
        })
        map.addLayer({
            'id': 'maine',
            'type': 'fill',
            'source': 'maine', // reference the data source
            'layout': {},
            'paint': {
                'fill-color': '#0080ff', // blue color fill
                'fill-opacity': 1
            }
        });
    })
    const update = (coords) => {
        map.getSource("maine").setData({
            'type': 'Feature',
            'geometry': {
                'type': 'Polygon',
                // These coordinates outline Maine.
                'coordinates': [coords]
            }
        })
    }
    // map.on("load", e => update(inite))
    const markers = inite.map(x => new mapboxgl.Marker({
        draggable: true,
    }).setLngLat(x).addTo(map))
    markers.forEach(x => {
        x.on("dragend", onDragEnd)
    })


    function onDragEnd() {
        const lngLat = markers.map(y => { let j = y.getLngLat(); return [j.lng, j.lat] })
        lngLat.push(lngLat[0])

        // inite = lngLat;
        // console.log(markers.map(y => { let j = y.getLngLat(); return [j.lng, j.lat] }))
        inside(markers.map(y => { let j = y.getLngLat(); return [j.lng, j.lat] }), lngLat)

        // console.log(lngLat)
        update(lngLat)
        // coordinates.innerHTML = `Longitude: ${lngLat.lng}<br />Latitude: ${lngLat.lat}`;
    }
    const point = new mapboxgl.Marker({
        draggable: true,
        color: "#b40219"
    }).setLngLat([-91.86713354492156, 42.74941219973357]).addTo(map)
    point.on("dragend", onMarkerEnd)
    function onMarkerEnd() {
        inside(markers.map(y => { let j = y.getLngLat(); return [j.lng, j.lat] }))
    }
    // marker.on('dragend', onDragEnd);
    const inside = (i, modified) => {
        const poly = new Quad(...i.map(x => new Point(x[0], x[1])))
        // console.log(poly.valid())

        const pot = new Point(point.getLngLat().lng, point.getLngLat().lat);

        let start = performance.now()
        const res = pip(poly, pot)
        document.getElementById("tim").innerText = `${performance.now() - start}`
        document.getElementById("pan").innerText = res


    }
}
main()