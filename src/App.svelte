<script lang="ts">
  import { event } from "@tauri-apps/api";

  let count = 0;
  event.listen("hello", () => {
    count++;
  });
</script>

<main class="container">
  <h1>tracing freeze repro</h1>

  <p>
    Events received: {count}
  </p>

  <p>Try resizing the window and see if it freezes.</p>
</main>
