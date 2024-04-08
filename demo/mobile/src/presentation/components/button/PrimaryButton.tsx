import {styled} from 'nativewind';
import React from 'react';
import {GestureResponderEvent, TouchableOpacity} from 'react-native';
import TextPrimary from '../text/TextPrimary';

const StyledTouchableOpacity = styled(TouchableOpacity);

interface PrimaryButtonProps {
  label: string;
  classStyle?: string;
  onPress: (event: GestureResponderEvent) => void;
}

const PrimaryButton: React.FC<PrimaryButtonProps> = ({
  label,
  onPress,
  classStyle,
}) => {
  return (
    <StyledTouchableOpacity
      onPress={onPress}
      className={'rounded-2xl bg-primary-normal w-full p-5 ' + classStyle}>
      <TextPrimary
        classStyle="text-center text-white"
        text={label}
        isBold={true}
      />
    </StyledTouchableOpacity>
  );
};

export default PrimaryButton;
