/**
 * Helper function for interacting with localStorage.
 */
export const localStorageHelper = {
  /**
   * Retrieves a value from localStorage by key.
   * @param key - The key to retrieve the value for.
   * @returns The value associated with the key, or null if the key doesn't exist.
   */
  getItem: (key: string): string | null => {
    try {
      const serializedValue = localStorage.getItem(key);
      if (serializedValue === null) return null;
      return JSON.parse(serializedValue);
    } catch (error) {
      console.error('Error retrieving from localStorage:', error);
      return null;
    }
  },

  /**
   * Sets a value in localStorage.
   * @param key - The key to set the value for.
   * @param value - The value to set.
   * @returns True if the value was successfully set, false otherwise.
   */
  setItem: (key: string, value: any): boolean => {
    try {
      const serializedValue = JSON.stringify(value);
      localStorage.setItem(key, serializedValue);
      return true;
    } catch (error) {
      console.error('Error setting value in localStorage:', error);
      return false;
    }
  },

  /**
   * Removes a value from localStorage by key.
   * @param key - The key to remove the value for.
   * @returns True if the value was successfully removed, false otherwise.
   */
  removeItem: (key: string): boolean => {
    try {
      localStorage.removeItem(key);
      return true;
    } catch (error) {
      console.error('Error removing value from localStorage:', error);
      return false;
    }
  },

  /**
   * Clears all values from localStorage.
   * @returns True if the operation was successful, false otherwise.
   */
  clear: (): boolean => {
    try {
      localStorage.clear();
      return true;
    } catch (error) {
      console.error('Error clearing localStorage:', error);
      return false;
    }
  },
};
