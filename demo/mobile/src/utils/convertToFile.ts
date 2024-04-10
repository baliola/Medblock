import {DocumentPickerResponse} from 'react-native-document-picker';
import {ImagePickerResponse} from 'react-native-image-picker';

export async function convertToFile(
  data: DocumentPickerResponse,
): Promise<File> {
  try {
    if (!data.uri || !data.name || !data.type) {
      throw new Error(
        'Invalid DocumentPickerResponse: Missing URI, name, or type',
      );
    }

    const response = await fetch(data.fileCopyUri ?? '-');

    if (response.status === 0) {
      throw new Error(
        'Network error: Failed to fetch file. Please check your internet connection.',
      );
    } else if (!response.ok) {
      throw new Error(
        `Failed to fetch file: ${response.status} ${response.statusText}`,
      );
    }

    const blob = await response.blob();
    const compressedBlob = await compressBlob(blob, data.fileCopyUri as string);

    const now = new Date().getTime();
    const fileName = `${data.name}`;
    return new File([compressedBlob as Blob], fileName, {
      type: data.type,
      lastModified: now,
    });
  } catch (error) {
    console.error('Error converting asset to File:', error);
    throw error;
  }
}

export async function convertAssetToFile(
  asset: ImagePickerResponse,
): Promise<File> {
  try {
    const uri = asset?.assets?.[0]?.uri;
    if (!uri) {
      throw new Error('URI is undefined');
    }
    const fetchedResponse = await fetch(uri);

    if (!fetchedResponse.ok) {
      throw new Error('Failed to load file');
    }
    const blob = await fetchedResponse.blob();
    const compressedBlob = await compressBlob(blob, uri);

    const now = new Date().getTime();
    const fileName = `${asset?.assets?.[0]?.fileName}`;
    return new File([compressedBlob as Blob], fileName, {
      type: asset?.assets?.[0]?.type as string,
      lastModified: now,
    });
  } catch (error) {
    console.error('Error converting asset to File:', error);
    throw error;
  }
}

export async function convertFileToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const base64String = reader.result as string;
      resolve(base64String.split(',')[1]); // remove the data prefix
    };
    reader.onerror = error => reject(error);
    reader.readAsDataURL(file);
  });
}

async function compressBlob(blob: Blob, uri?: string): Promise<Blob | null> {
  return new Promise<Blob | null>(async resolve => {
    resolve(blob);
  });
}

export async function convertUriToFile(
  uri: string,
  name?: string | null,
): Promise<File> {
  const response = await fetch(uri);
  const blob = await response.blob();
  const file = new File(
    [blob],
    `${name ?? ''}-${new Date().getTime()}.${blob.type.split('/')[1]}`,
    {
      type: blob.type,
      lastModified: new Date().getTime(),
    },
  );

  return file;
}
