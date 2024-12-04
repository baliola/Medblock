import { Button, Drawer, DrawerBody, DrawerContent, DrawerFooter, DrawerOverlay } from "@chakra-ui/react";

interface PinShareConcentCheckerProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: () => void;
}

export default function PinShareConcentChecker({
  isOpen,
  onClose,
  onSubmit
}: PinShareConcentCheckerProps) {
  return (
    <Drawer
      isOpen={isOpen}
      placement="bottom"
      onClose={onClose}
    >
      <DrawerOverlay />
      <DrawerContent>
        <DrawerBody textAlign={'center'}>
          Do you want make your Login Account Safety with 6 Digit Pins?
        </DrawerBody>
        <DrawerFooter alignItems={'center'} justifyContent={'center'} gap={3}>
          <Button
            variant={'ghost'}
            color={'primary.700'}
            onClick={onClose}
          >
            Dont Need
          </Button>
          <Button colorScheme="primary"
            bg={"primary.700"}
            onClick={onSubmit}
          >
            Sure
          </Button>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  )
}