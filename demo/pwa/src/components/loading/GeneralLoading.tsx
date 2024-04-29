import React, { FC } from 'react';

import DialogBasic from '../Dialog/DialogBasic';

interface GeneralLoadingProps {
  loading: boolean;
}

const GeneralLoading: FC<GeneralLoadingProps> = ({ loading }) => {
  return (
    <DialogBasic
      open={loading}
      child={
        <div className="flex flex-col items-center justify-center p-4">
          <img src="/assets/logo.svg" alt="" />
          <p className="text-lg text-gray-800 font-bold mt-2">Loading...</p>
        </div>
      }
    />
  );
};

export default GeneralLoading;
