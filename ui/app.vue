<script setup>
import init, {run} from 'app';

onMounted(async () => {
  await init();
  run();
});

</script>
<template>
  <div>
    <NuxtRouteAnnouncer />
    <NuxtWelcome />
  </div>
</template>
