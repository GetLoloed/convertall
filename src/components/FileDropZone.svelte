<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { open } from '@tauri-apps/api/dialog';
    import { filesStore } from '../stores/store';

    const dispatch = createEventDispatcher();
    let files: { name: string, path: string, file: File }[] = [];
    let dragover = false;

    function isImageFile(file: File): boolean {
        return file.type.startsWith('image/');
    }

    function handleDrop(e: DragEvent) {
        e.preventDefault();
        dragover = false;
        const droppedFiles = Array.from(e.dataTransfer?.files || []);
        addFiles(droppedFiles.filter(isImageFile));
    }

    async function handleFileInput() {
        try {
            const selected = await open({
                multiple: true,
                filters: [{
                    name: 'Image',
                    extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp']
                }]
            });

            if (Array.isArray(selected)) {
                const newFiles = selected.map(filePath => ({
                    name: filePath.split('\\').pop() || filePath.split('/').pop() || filePath,
                    path: filePath
                }));
                filesStore.update(currentFiles => [...currentFiles, ...newFiles]);
            }
        } catch (error) {
            console.error("Erreur lors de la sélection des fichiers:", error);
        }
    }

    function addFiles(newFiles: File[]) {
        files = [...files, ...newFiles];
        dispatch("filesAdded", files);
    }

    function removeFile(index: number) {
        files = files.filter((_, i) => i !== index);
        dispatch("filesAdded", files);
    }
</script>

<div
    class="mt-1 flex justify-center px-6 pt-5 pb-6 border-2 border-gray-300 border-dashed rounded-md hover:border-indigo-500 transition-colors duration-200"
    class:border-indigo-500={dragover}
    on:click={() => document.getElementById("fileInput")?.click()}
    on:keydown={(e) => e.key === "Enter" && document.getElementById("fileInput")?.click()}
    tabindex="0"
    role="button"
    aria-label="Sélectionner ou déposer des images"
    on:dragover|preventDefault={() => (dragover = true)}
    on:dragleave={() => (dragover = false)}
    on:drop={handleDrop}
>
    <div class="space-y-1 text-center">
        <svg
            class="mx-auto h-12 w-12 text-gray-400"
            stroke="currentColor"
            fill="none"
            viewBox="0 0 48 48"
            aria-hidden="true"
        >
            <path
                d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
        <div class="flex text-sm text-gray-600">
            <label
                for="fileInput"
                class="relative cursor-pointer bg-white rounded-md font-medium text-indigo-600 hover:text-indigo-500 focus-within:outline-none focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-indigo-500"
            >
                <span>Sélectionnez des images</span>
                <input
                    id="fileInput"
                    type="file"
                    multiple
                    accept="image/*"
                    class="sr-only"
                    on:change={handleFileInput}
                />
            </label>
            <p class="pl-1">ou déposez-les ici</p>
        </div>
        <p class="text-xs text-gray-500">Tous formats d'image acceptés</p>
    </div>
</div>

{#if files.length > 0}
    <ul class="mt-4 space-y-2">
        {#each files as file, index}
            <li
                class="flex items-center justify-between bg-gray-50 px-4 py-2 rounded-md"
            >
                <span class="text-sm text-gray-600">{file.name}</span>
                <button
                    on:click={() => {
                        files = files.filter((_, i) => i !== index);
                        dispatch("filesAdded", files);
                    }}
                    class="text-red-500 hover:text-red-700 focus:outline-none"
                >
                    ❌
                </button>
            </li>
        {/each}
    </ul>
{/if}