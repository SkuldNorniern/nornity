<template>
    <NuxtLayout>
      <NuxtPage/>
    </NuxtLayout>
</template>

<script lang="ts">
import useDarkMode from '~/composables/useDarkMode';
import 'assets/global.css';

export default {
  setup() {
    const { isDarkMode, toggleDarkMode } = useDarkMode();
    return { isDarkMode, toggleDarkMode };
  },
};
</script>

