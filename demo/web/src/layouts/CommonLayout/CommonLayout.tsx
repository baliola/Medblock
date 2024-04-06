import Image from 'next/image';
import { FC, ReactElement } from 'react';

import styles from './CommonLayout.module.css';

interface CommonLayoutProps {
  children: ReactElement;
}

export const CommonLayout: FC<CommonLayoutProps> = ({ children }) => {
  return (
    <div className="">
      <main className="">{children}</main>
    </div>
  );
};

const getLayout = (page: ReactElement) => <CommonLayout>{page}</CommonLayout>;

export default getLayout;
