import React from 'react';
import {styled} from 'nativewind';
import {View} from 'react-native';
import Dialog from 'react-native-dialog';
import Colors from '@constants/colors';
import TextPrimary from '@components/text/TextPrimary';
import {useTranslation} from 'react-i18next';
import BasicButton from '@components/button/BasicButton';
import BorderRoundedButton from '@components/button/BorderRoundedButton';
const StyledView = styled(View);

interface ModalDialogProps {
  show: boolean;
  title?: string;
  child?: React.ReactElement<any, any>;
  onClose: () => void;
  labelRight?: string;
  labelCancel?: string;
  onRightTap?: () => void;
}

const ModalDialog: React.FC<ModalDialogProps> = ({
  show,
  title,
  child,
  onClose,
  onRightTap,
  labelCancel,
  labelRight,
}) => {
  const {t} = useTranslation('global');

  return (
    <Dialog.Container
      onBackdropPress={onClose}
      visible={show}
      headerStyle={{display: 'none'}}
      contentStyle={{
        borderRadius: 30,
        elevation: 0,
        backgroundColor: Colors.white,
      }}>
      <StyledView className="flex flex-col justify-between">
        <StyledView className="h-auto w-full rounded-xl flex justify-center items-center mb-4">
          <TextPrimary
            text={title}
            classStyle="text-gray-800 p-4"
            style={{fontSize: 16}}
            isBold={true}
          />
          {child}
        </StyledView>

        <StyledView className="flex flex-row justify-center space-x-2 p-4">
          <BasicButton
            label={t(labelRight as string)}
            onPress={() => {
              onRightTap!();
            }}
            classStyle="w-1/3 py-2 rounded-xl"
            labelStyle={{color: Colors.white}}
            style={{backgroundColor: Colors.red}}
          />
          <BorderRoundedButton
            label={t(labelCancel as string)}
            onPress={onClose}
            classStyle="w-1/3 py-2 rounded-xl"
            labelStyle={{color: Colors.gray_dark}}
          />
        </StyledView>
      </StyledView>
    </Dialog.Container>
  );
};

export default ModalDialog;
