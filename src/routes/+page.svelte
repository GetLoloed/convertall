<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { join, basename, dirname } from "@tauri-apps/api/path";
  import { filesStore, selectedFormatStore } from '../stores/store';
  import FileDropZone from '../components/FileDropZone.svelte';
  import ConversionOptions from '../components/ConversionOptions.svelte';
  import { writable } from 'svelte/store';

  let convertedFiles: { original: string, converted: string }[] = [];
  let logs: string[] = [];
  let isConverting = false;
  let progress = writable(0);

  async function convertFiles() {
    isConverting = true;
    progress.set(0);
    const totalFiles = $filesStore.length;

    for (let i = 0; i < totalFiles; i++) {
      const file = $filesStore[i];
      try {
        console.log("Traitement du fichier:", file.name);
        const inputPath = file.path;
        
        if (!inputPath) {
          throw new Error("Le chemin du fichier est manquant");
        }

        const fileName = await basename(inputPath);
        const originalDir = await dirname(inputPath);
        const outputPath = await join(originalDir, `converted-${fileName.replace(/\.[^/.]+$/, '')}.${$selectedFormatStore}`);
        
        console.log("Appel de la fonction de conversion");
        const result = await invoke('convert_image', { 
          inputPath, 
          outputPath, 
          format: $selectedFormatStore 
        });
        
        console.log("Résultat de la conversion:", result);

        if (result.success) {
          convertedFiles = [...convertedFiles, { 
            original: result.path,
            converted: result.output_path || ''
          }];
          logs = [...logs, result.message];
        } else {
          throw new Error(result.message || "Erreur inconnue lors de la conversion");
        }
      } catch (error) {
        console.error(`Erreur lors de la conversion de ${file.name}:`, error);
        logs = [...logs, `Erreur: ${error}`];
      }

      progress.set(((i + 1) / totalFiles) * 100);
    }

    isConverting = false;
  }

  function handleFormatSelected(event: CustomEvent<string>) {
    selectedFormatStore.set(event.detail);
  }

  function removeFile(index: number) {
    $filesStore = $filesStore.filter((_, i: number) => i !== index);
  }
</script>

<main class="container mx-auto p-4">
  <h1 class="text-3xl font-bold mb-6">Convertisseur d'images</h1>
  
  <div class="mb-6">
    <FileDropZone />
  </div>

  <div class="mb-6">
    <ConversionOptions on:formatSelected={handleFormatSelected} />
  </div>

  <ul class="mb-6">
    {#each $filesStore as file, index}
      <li class="flex items-center justify-between py-2">
        <span>{file.name}</span>
        <button on:click={() => removeFile(index)} class="text-red-500">❌</button>
      </li>
    {/each}
  </ul>

  <button 
    on:click={convertFiles}
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
    disabled={isConverting}
  >
    Convertir les fichiers
  </button>
  
  {#if isConverting}
    <div class="mt-4 w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
      <div class="bg-blue-600 h-2.5 rounded-full" style="width: {$progress}%"></div>
    </div>
  {/if}

  <div class="mt-8">
    <h2 class="text-2xl font-semibold mb-4">Logs :</h2>
    <ul class="bg-gray-100 p-4 rounded">
      {#each logs as log}
        <li class="mb-2">{log}</li>
      {/each}
    </ul>
  </div>
  
  <div class="mt-8">
    <h2 class="text-2xl font-semibold mb-4">Fichiers convertis :</h2>
    <ul class="bg-gray-100 p-4 rounded">
      {#each convertedFiles as file}
        <li class="mb-2">
          <span class="font-medium">Original:</span> {file.original} 
          <span class="font-medium ml-2">Converti:</span> {file.converted}
        </li>
      {/each}
    </ul>
  </div>
</main>