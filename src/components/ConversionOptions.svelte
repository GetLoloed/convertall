<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let selectedFormat = '';
  
  const dispatch = createEventDispatcher<{formatSelected: string}>();
  
  const imageFormats = ['jpg', 'png', 'webp', 'gif', 'tiff', 'bmp'];
  
  function handleFormatChange() {
    dispatch('formatSelected', selectedFormat);
  }
</script>
  
<div class="conversion-options">
  <div class="mt-8">
    <label for="format-select" class="block text-sm font-medium text-gray-700">Choisissez le format de conversion :</label>
    <select
      id="format-select"
      bind:value={selectedFormat}
      on:change={handleFormatChange}
      class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
    >
      <option value="">Sélectionnez un format</option>
      {#each imageFormats as format}
        <option value={format}>.{format}</option>
      {/each}
    </select>
  </div>
</div>
  
<style>
  .conversion-options {
    margin-top: 2rem;
  }
  
  select {
    width: 100%;
    padding: 0.5rem;
    font-size: 1rem;
  }
</style>