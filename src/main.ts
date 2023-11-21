import { createApp } from "vue";
import App from "./App.vue";
import "./styles.css";

import "primevue/resources/primevue.min.css";
import "primevue/resources/themes/md-dark-indigo/theme.css"

import "primeflex/primeflex.css"
import "primeflex/themes/primeone-dark.css"

import 'primeicons/primeicons.css'

import Splitter from 'primevue/splitter';
import SplitterPanel from "primevue/splitter";
import PrimeVue from 'primevue/config';
import Sidebar from "primevue/sidebar";


const app = createApp(App);
app.component("Splitter", Splitter);
app.component("SplitterPanel", SplitterPanel);
app.component("Sidebar", Sidebar);
app.component("SplitterPanel", SplitterPanel);
app.use(PrimeVue);
app.mount("#app");
