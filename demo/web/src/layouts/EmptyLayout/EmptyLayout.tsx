import Image from 'next/image';
import React, { FC, ReactElement, useState } from 'react';
import { ToastContainer } from 'react-toastify';

export interface Props {
  children: ReactElement;
}

export const EmptyLayout: FC<Props> = ({ children }) => {
  return (
    <main className="flex justify-items-center  bg-no-repeat w-full bg-cover items-center min-h-screen p-8 my-auto bg-bg-primary text-text-primary ">
      {children}
      <ToastContainer theme="dark" />
    </main>
  );
};

const getLayout = (page: ReactElement) => <EmptyLayout>{page}</EmptyLayout>;

export default getLayout;
