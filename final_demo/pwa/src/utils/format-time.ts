export const convertBigIntToTime = (nanoseconds: bigint) => {
  // Convert nanoseconds to milliseconds
  const milliseconds = Number(nanoseconds) / 1_000_000;

  // Create a Date object with the milliseconds
  const date = new Date(milliseconds);

  // Format the date to a readable string
  // You can customize this to your preferred format
  const formattedDate = date.toLocaleString('en-US', {
    weekday: 'short',
    day: 'numeric',
    month: 'short',
    year: 'numeric',
    hour: 'numeric',
    minute: 'numeric',
  });

  return formattedDate;
}