import "./Try.css";
import Try from "./Try.svelte";

const app = new Try({
  target: document.getElementById("app"),
  props: {
    apiURL: "http://localhost:1971",
  },
});

export default app;
