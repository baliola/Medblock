import GeneralImage from '@components/image/GeneralImage';
import {WIDTH} from '@constants/dimensions';
import Images from '@constants/images';
import {styled} from 'nativewind';
import React from 'react';
import {View} from 'react-native';
import AppBar from './AppBar';
import TextPrimary from '@components/text/TextPrimary';

const StyledView = styled(View);

interface AppBarWithIconProps {
  title?: string;
  titleClassStyle?: string;
  titleBold?: boolean;
  child?: React.ReactElement<any, any>;
}

const AppBarWithIcon: React.FC<AppBarWithIconProps> = ({
  title,
  child,
  titleClassStyle,
  titleBold,
}) => {
  return (
    <StyledView className="flex flex-col">
      <GeneralImage
        url={Images.logo3}
        size={WIDTH}
        classStyle="absolute -top-28"
      />
      <AppBar
        style={{marginTop: 100}}
        title={
          title ? (
            <TextPrimary
              text={title}
              classStyle={'text-gray-800 ' + titleClassStyle}
              isBold={titleBold}
            />
          ) : null
        }
      />

      {child}
    </StyledView>
  );
};

export default AppBarWithIcon;
