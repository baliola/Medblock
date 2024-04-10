import {styled} from 'nativewind';
import React from 'react';
import {Image, View} from 'react-native';

const StyledView = styled(View);
const StyledImage = styled(Image);

interface GeneralImageProps {
  url: string;
  size?: number;
  classStyle?: string;
  tintColor?: string;
}

const GeneralImage: React.FC<GeneralImageProps> = ({
  size,
  url,
  classStyle,
  tintColor,
}) => {
  return (
    <StyledView
      style={{height: size ?? 'auto', width: size ?? 'auto'}}
      className={classStyle}>
      <StyledImage
        source={parseInt(url, 10)}
        className="h-full w-full"
        style={{objectFit: 'scale-down', tintColor: tintColor}}
      />
    </StyledView>
  );
};

export default GeneralImage;
