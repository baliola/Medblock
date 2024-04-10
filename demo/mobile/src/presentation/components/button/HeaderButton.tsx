import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import Images from '@constants/images';
import {styled} from 'nativewind';
import React from 'react';
import {TouchableOpacity, View} from 'react-native';

const StyledView = styled(View);
const StyledTouchableOpacity = styled(TouchableOpacity);

interface HeaderButtonProps {
  icon: string;
  label: string;
  onPress: () => void;
  classStyle?: string;
}

const HeaderButton: React.FC<HeaderButtonProps> = ({
  icon,
  label,
  onPress,
  classStyle,
}) => {
  return (
    <StyledTouchableOpacity
      onPress={() => {
        onPress();
      }}
      className={'flex flex-row justify-between items-center ' + classStyle}>
      <StyledView className="flex flex-row space-x-2">
        <GeneralImage url={icon} size={24} />
        <TextPrimary text={label} classStyle="text-gray-800 text-xl" isBold />
      </StyledView>
      <GeneralImage url={Images.arrowRight} size={22} />
    </StyledTouchableOpacity>
  );
};

export default HeaderButton;
