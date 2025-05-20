from transformers import SpeechT5Processor, SpeechT5ForTextToSpeech, SpeechT5HifiGan
from datasets import load_dataset
import torch

device = "cuda" if torch.cuda.is_available() else "cpu"

MODEL = "microsoft/speecht5_tts"
processor = SpeechT5Processor.from_pretrained(MODEL)
model = SpeechT5ForTextToSpeech.from_pretrained(MODEL).to(device)
vocoder = SpeechT5HifiGan.from_pretrained("microsoft/speecht5_hifigan").to(device)

embeddings_dataset = load_dataset("Matthijs/cmu-arctic-xvectors", split="validation")
speaker_embeddings = torch.tensor(embeddings_dataset[7306]["xvector"]).unsqueeze(0).to(device)

def text_to_speech(text):
    inputs = processor(text=text, return_tensors="pt").to(device)
    speech = model.generate_speech(inputs["input_ids"], speaker_embeddings, vocoder=vocoder)

    return speech.cpu().numpy()