import Alpine from 'alpinejs';
import Htmx from 'htmx.org';
import Leaflet from 'leaflet';

import '@picocss/pico/css/pico.css';
import 'leaflet/dist/leaflet.css';

window.Alpine = Alpine;
window.htmx = Htmx;
window.L = Leaflet;

document.dispatchEvent(new Event('ready'));