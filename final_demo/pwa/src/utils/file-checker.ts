interface FileCheckerProps {
  file: File;
  allowedTypes: string[];
  maxSize: number;
};

export const imageTypes = [
  "image/jpeg",
  "image/png",
  "image/jpg",
];

export const fileChecker = ({
  file,
  allowedTypes,
  maxSize,
}: FileCheckerProps) => {
  if (!allowedTypes.includes(file.type)) {
    throw new Error("File type is not allowed");
  }

  if (file.size > maxSize) {
    throw new Error(`File size is too large, maximum is ${maxSize / 1024 / 1024} MB`);
  }

  return true;
}