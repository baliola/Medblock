import { useRouter } from 'next/router';
import React, { CSSProperties, FC } from 'react';
import { ArrowLeft } from 'solar-icon-set';

interface AppBarProps {
  title?: React.ReactElement<any, any> | null;
  inverse?: boolean;
  trailing?: React.ReactElement<any, any> | null;
  classStyle?: string;
  style?: CSSProperties;
}

const AppBar: FC<AppBarProps> = ({
  classStyle,
  inverse,
  style,
  title,
  trailing,
}) => {
  const navigation = useRouter();

  const handleGoBack = () => {
    navigation.back();
  };

  return (
    <div
      style={style}
      className={
        'px-4 pt-6 flex flex-row justify-between items-center w-full bg-white ' +
        classStyle
      }
    >
      {title ? (
        <button onClick={handleGoBack} className="h-7 w-7">
          <ArrowLeft color="black" iconStyle="Bold" size={20} />
        </button>
      ) : null}

      <div className="flex-1 pl-4">{title}</div>
      <div className="px-2">{trailing}</div>
    </div>
  );
};

export default AppBar;
