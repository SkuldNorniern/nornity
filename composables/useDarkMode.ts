import { ref, onMounted, watch } from 'vue';

export default function useDarkMode() {
    const isDarkMode = ref(false);

    onMounted(() => {
        // Explicitly check for null to ensure compatibility
        const storedDarkMode = localStorage.getItem('darkMode');
        if (storedDarkMode !== null) {
            isDarkMode.value = storedDarkMode === 'true';
        } else {
            // Optionally, use a default mode based on system preferences
            // This is useful for first-time visitors
            isDarkMode.value = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
        }
        updateClassList(isDarkMode.value);
    });

    watch(isDarkMode, (newValue) => {
        updateClassList(newValue);
        localStorage.setItem('darkMode', newValue ? 'true' : 'false');
    });

    const toggleDarkMode = () => {
        isDarkMode.value = !isDarkMode.value;
    };

    function updateClassList(isDark: boolean) {
        const bodyClassList = document.body.classList;
        bodyClassList.remove('chakra-ui-light'); // Always remove 'chakra-ui-light' to prevent interference

        if (isDark) {
            bodyClassList.add('dark-mode');
        } else {
            bodyClassList.remove('dark-mode');
        }
    }

    return { isDarkMode, toggleDarkMode };
}