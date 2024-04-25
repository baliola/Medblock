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
