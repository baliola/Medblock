export const calculateAge = (birthDate: string): string => {
  const birth = new Date(birthDate);
  const today = new Date();

  let ageInYears = today.getFullYear() - birth.getFullYear();

  if (
    today < new Date(
      today.getFullYear(),
      birth.getMonth(),
      birth.getDate()
    )
  ) {
    ageInYears--;
  }

  if (ageInYears < 1) {
    let months = today.getMonth() - birth.getMonth() + (12 * (today.getFullYear() - birth.getFullYear()));

    if (months < 0) {
      months = 0;
    }

    return `${months} Bln`;
  }

  if (ageInYears < 0) {
    ageInYears = 0;
  }

  return `${ageInYears} Th`;
}
