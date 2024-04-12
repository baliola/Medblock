import React from 'react';

import Images from '@/constants/images';

import AppBar from './AppBar';

interface AppBarWithIconProps {
  title: React.ReactElement<any, any>;
  titleClassStyle?: string;

  child?: React.ReactElement<any, any>;
}

const AppBarWithIcon: React.FC<AppBarWithIconProps> = ({
  title,
  child,
  titleClassStyle,
}) => {
  return (
    <div className="flex flex-col bg-white">
      <img src={Images.medblock} alt="" className="w-full" />
      <AppBar title={title ? title : null} />
      {child}
    </div>
  );
};

export default AppBarWithIcon;
