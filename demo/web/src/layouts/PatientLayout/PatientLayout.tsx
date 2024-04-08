import Image from 'next/image';
import Link from 'next/link';
import React, { FC, ReactElement, useState } from 'react';
import { ToastContainer } from 'react-toastify';

export interface Props {
  children: ReactElement;
}

export const PatientLayout: FC<Props> = ({ children }) => {
  return (
    <div className="flex flex-col p-4">
      <main className="flex bg-no-repeat w-full bg-cover min-h-screen my-auto">
        {children}
        <ToastContainer theme="dark" />
      </main>
    </div>
  );
};

const getLayout = (page: ReactElement) => <PatientLayout>{page}</PatientLayout>;

export default getLayout;
