const dateFormatter = (date: Date) => {
  const locale = 'en-US';

  const formattedDate = date.toLocaleDateString(locale, {
    day: '2-digit',
    year: 'numeric',
    month: 'short',
    hour: '2-digit',
    minute: '2-digit',
  });

  return formattedDate;
};

export default dateFormatter;
