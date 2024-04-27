export const formatDateFromBigInt = (timestamp: bigint): string => {
  // Convert the BigInt to a number
  const second = Number(timestamp);

  console.log('second', second);
  const timeInSeconds = Math.floor(second / 1e6);

  // Create a Date object from the milliseconds
  const date = new Date(timeInSeconds);
  console.log('date', date);

  // Get day, month, and year
  const day = ('0' + date.getDate()).slice(-2);
  const month = ('0' + (date.getMonth() + 1)).slice(-2);
  const year = date.getFullYear();

  // Format the date as dd/mm/yyyy
  const formattedDate = `${day}/${month}/${year}`;

  return formattedDate;
};

export const formatDateFromBigIntWithTime = (timestamp: bigint): string => {
  // Convert the BigInt to a number
  const second = Number(timestamp);

  // Convert nanoseconds to milliseconds
  const timeInSeconds = Math.floor(second / 1e6);

  // Create a Date object from the milliseconds
  const date = new Date(timeInSeconds);

  // Get day, month, and year
  const day = ('0' + date.getDate()).slice(-2);
  const month = ('0' + (date.getMonth() + 1)).slice(-2);
  const year = date.getFullYear();

  // Get hours and minutes
  const hours = ('0' + date.getHours()).slice(-2);
  const minutes = ('0' + date.getMinutes()).slice(-2);

  // Format the date as dd/mm/yyyy hh:mm
  const formattedDate = `${day}/${month}/${year} ${hours}:${minutes}`;

  return formattedDate;
};
