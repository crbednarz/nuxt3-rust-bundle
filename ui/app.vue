<script setup>
import init, {test} from 'app';

onBeforeMount(async () => {
  await init();
  alert(test());
});

</script>
<template>
  <div>
    <NuxtRouteAnnouncer />
    <NuxtWelcome />
  </div>
</template>
