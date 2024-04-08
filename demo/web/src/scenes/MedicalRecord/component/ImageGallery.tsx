import Link from 'next/link';
import React, { Dispatch, SetStateAction, useState } from 'react';

interface Modals {
  setShowModal: Dispatch<SetStateAction<boolean>>;
  setSelectedImage: Dispatch<SetStateAction<string>>;
  //   toggle: (image: string) => void;
}

const ImageGallery = (props: Modals) => {
  const { setSelectedImage, setShowModal } = props;
  const [images, setImages] = useState([
    'https://flowbite.s3.amazonaws.com/docs/gallery/square/image-1.jpg',
    'https://flowbite.s3.amazonaws.com/docs/gallery/square/image-2.jpg',
    'https://flowbite.s3.amazonaws.com/docs/gallery/square/image-3.jpg',
    'https://flowbite.s3.amazonaws.com/docs/gallery/square/image-4.jpg',
    'https://flowbite.s3.amazonaws.com/docs/gallery/square/image-5.jpg',
    'https://flowbite.s3.amazonaws.com/docs/gallery/square/image-6.jpg',
  ]);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);

  const [fileInputVisible, setFileInputVisible] = useState(false);
  const shouldScroll = images.length > 5;
  const toggleModal = (imageUrl: string) => {
    setSelectedImage(imageUrl);
    setShowModal(true);
  };

  const handleImageUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      setSelectedFile(file);
      const url = URL.createObjectURL(file);
      setImages([...images, url]);
    }
  };

  return (
    <div className="flex overflow-x-auto gap-4">
      {images.map((imageUrl, index) => (
        <div key={index}>
          {/* <Link href={imageUrl}> */}
          {index === images.length - 1 ? (
            <div className="flex gap-4">
              {selectedFile && (
                <div className="relative">
                  <img
                    className="h-24 max-w-[120px] rounded-2xl cursor-pointer"
                    src={URL.createObjectURL(selectedFile)}
                    alt="Image Preview"
                    onClick={() => toggleModal(imageUrl)}
                  />
                  <div className="absolute inset-0 flex items-center rounded-2xl justify-center bg-black bg-opacity-50">
                    <span className="text-white">Preview</span>
                  </div>
                </div>
              )}
              <div className="flex items-center justify-center w-[120px] h-24">
                <label
                  htmlFor={'dropzone-file'}
                  className="flex flex-col h-full items-center justify-center w-full border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
                >
                  <div className="flex flex-col items-center justify-center gap-2">
                    <svg
                      className="w-8 h-8text-gray-500 dark:text-gray-400"
                      aria-hidden="true"
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 20 16"
                    >
                      <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"
                      />
                    </svg>
                    <p className=" text-xs text-center text-gray-500 dark:text-gray-400">
                      <span className="font-semibold">Click to upload</span> or
                      drag and drop
                    </p>
                  </div>
                  <input
                    id="dropzone-file"
                    type="file"
                    className="hidden"
                    onChange={handleImageUpload}
                  />
                </label>
              </div>
            </div>
          ) : (
            <div key={index}>
              <img
                className="h-24 max-w-[120px] rounded-lg"
                src={imageUrl}
                alt={`Image ${index + 1}`}
                onClick={() => toggleModal(imageUrl)}
              />
            </div>
          )}
        </div>
      ))}
    </div>
  );
};

export default ImageGallery;
